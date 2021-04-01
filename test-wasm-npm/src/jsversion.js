/**
 * 是否是 app 中
 * @param  {String}  userAgent
 * @param  {String}  type  (link/lianjia/deyou/baichuan/beike)
 * @return {Boolean}
 */
 module.exports = function isAppWebView (userAgent, type) {
  type = String(type).toLowerCase()
  switch (type) {
    case 'link':     return isLink(userAgent)
    case 'lianjia':  return isLianjia(userAgent)
    case 'deyou':    return isDeyou(userAgent)
    case 'baichuan': return isBaichuan(userAgent)
    case 'beike':    return isBeike(userAgent)
    case 'atom':     return isAtom(userAgent)
    case 'linkxinfang': return isLinkXinfang(userAgent)
    case 'vrstudio': return isVRStudio(userAgent)
    default:         return isLink(userAgent) ||
                            isLianjia(userAgent) ||
                            isDeyou(userAgent) ||
                            isBaichuan(userAgent) ||
                            isBeike(userAgent) ||
                            isAtom(userAgent) ||
                            isLinkXinfang(userAgent) ||
                            isVRStudio(userAgent)
  }
}

/**
 * 是否是 link 中
 * @param  {String}  userAgent
 * @return {Boolean}
 */
function isVRStudio (userAgent) {
  return /VRStudio\s*([0-9a-z\.\-]+)/i.test(userAgent)
}
/**
 * 是否是 link 中
 * @param  {String}  userAgent
 * @return {Boolean}
 */
function isLink (userAgent) {
  return /Lianjia\/HomeLink\/([0-9a-z\.\-]+)/i.test(userAgent)
}
/**
 * 是否是 link新房 中
 * @param  {String}  userAgent
 * @return {Boolean}
 */
function isLinkXinfang(userAgent) {
  return /HomeLink\/([0-9a-z\.\-]+)/i.test(userAgent)
}

/**
 * 是否是 德祐 中
 * @param  {String}  userAgent
 * @return {Boolean}
 */
function isDeyou (userAgent) {
  return /Lianjia\/Alliance\/?([0-9a-z\.\-]+)/i.test(userAgent) || /LianjiaAlliance\/?([0-9a-z\.\-]+)/i.test(userAgent)
}

/**
 * 是否是 百川 中
 * @param  {String}  userAgent
 * @return {Boolean}
 */
function isBaichuan (userAgent) {
  return /Lianjia\/lianjiabaichuan\/([0-9a-z\.\-]+)/i.test(userAgent)
}

/**
 * 是否是 贝壳 中
 * @param  {String}  userAgent
 * @return {Boolean}
 */
function isBeike (userAgent) {
  return /lianjiabeike\/([0-9a-z\.\-]+)/i.test(userAgent)
}

/**
 * 是否是 房江湖 中
 * @param  {String}  userAgent
 * @return {Boolean}
 */
function isAtom (userAgent) {
  return /lianjiaatom\/([0-9a-z\.\-]+)/i.test(userAgent)
}

/**
 * 是否是 掌链 中
 * @param  {String}  userAgent
 * @return {Boolean}
 */
function isLianjia (userAgent) {
  if (/Lianjia/i.test(userAgent)) {
    return !(
      isDeyou(userAgent) ||
      isBaichuan(userAgent) ||
      isBeike(userAgent) ||
      isLink(userAgent) ||
      isAtom(userAgent)
    )
  }
  return false
}
